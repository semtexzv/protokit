#include "absl/log/absl_log.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "conformance/conformance.pb.h"
#include "conformance_test.h"
#include "test_runner.h"

using conformance::ConformanceResponse;
using google::protobuf::ConformanceTestSuite;
using std::string;
using std::vector;

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
#include <fstream>
#include <map>
#include <algorithm>

extern "C" {

uint32_t run_rust(
    const uint8_t* data,
    uint32_t len,
    uint8_t* odata,
    uint32_t olen);

} // extern "C"


class rust_runner : public google::protobuf::ConformanceTestRunner {
 public:
  virtual std::string RunTest(absl::string_view test_name,
                              absl::string_view input) override {
    // fprintf(stderr, "Running test: %s\n", string(test_name).c_str());
    uint8_t data[8192];
    uint32_t out_len =
        run_rust((const uint8_t*) input.data(), input.length(), data, 8192);
    return string((char*) data, (size_t) out_len);
  }


  static void ParseFailureList(const char *filename,
                               conformance::FailureSet *failure_list) {
    std::ifstream infile(filename);

    if (!infile.is_open()) {
      fprintf(stderr, "Couldn't open failure list file: %s\n", filename);
      exit(1);
    }

    for (std::string line; std::getline(infile, line);) {
      // Remove comments.
      std::string test_name = line.substr(0, line.find('#'));

      test_name.erase(
          std::remove_if(test_name.begin(), test_name.end(), ::isspace),
          test_name.end());

      if (test_name.empty()) {  // Skip empty lines.
        continue;
      }

      // If we remove whitespace from the beginning of a line, and what we have
      // left at first is a '#', then we have a comment.
      if (test_name[0] != '#') {
        // Find our failure message if it exists. Will be set to an empty string
        // if no message is found. Empty failure messages also pass our tests.
        size_t check_message = line.find('#');
        std::string message;
        if (check_message != std::string::npos) {
          message = line.substr(check_message + 1);  // +1 to skip the delimiter
          // If we had only whitespace after the delimiter, we will have an empty
          // failure message and the test will still pass.
          // Note: absl::StripAsciiWhitespace is not available here easily without full absl include,
          // checking conformance_test_runner.cc used it. We'll simplify or use what's available.
          // Assuming simple trim is enough or just ignoring message content matching for now.
        }
        conformance::TestStatus *test = failure_list->add_test();
        test->set_name(test_name);
        test->set_failure_message(message);
      }
    }
  }

  static int Run(int argc, char* argv[],
                 const std::vector<ConformanceTestSuite*>& suites) {
    if (suites.empty()) {
      fprintf(stderr, "No test suites found.\n");
      return EXIT_FAILURE;
    }

    std::map<std::string, std::string> failure_list_files;
    for (int i = 1; i < argc; ++i) {
        // Simple arg parsing assuming --flag filename
        if (argv[i][0] == '-' && i + 1 < argc) {
             failure_list_files[argv[i]] = argv[i+1];
             i++;
        }
    }

    bool all_ok = true;
    for (ConformanceTestSuite* suite: suites) {
      std::vector<string> program_args;
      string failure_list_filename;
      conformance::FailureSet failure_list;
      
      std::string flag = suite->GetFailureListFlagName();
      if (failure_list_files.count(flag)) {
          failure_list_filename = failure_list_files[flag];
          ParseFailureList(failure_list_filename.c_str(), &failure_list);
      }

      bool performance = false;
      suite->SetOutputDir(".");
      suite->SetVerbose(false);
      rust_runner runner{};

      std::string output;
      all_ok =
          all_ok && suite->RunSuite(&runner, &output, failure_list_filename,
                                    &failure_list);

      fwrite(output.c_str(), 1, output.size(), stderr);
    }
    return all_ok ? EXIT_SUCCESS : EXIT_FAILURE;
  }

};

#include "binary_json_conformance_suite.h"
#include "conformance_test.h"
#include "text_format_conformance_suite.h"

int main(int argc, char* argv[]) {
  google::protobuf::BinaryAndJsonConformanceSuite binary_and_json_suite;
  google::protobuf::TextFormatConformanceTestSuite text_format_suite;
  return rust_runner::Run(
      argc, argv, {&binary_and_json_suite, &text_format_suite});
}
