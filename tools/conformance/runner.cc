#include "absl/log/absl_log.h"
#include "absl/strings/str_format.h"
#include "conformance/conformance.pb.h"
#include "conformance_test.h"

using conformance::ConformanceResponse;
using google::protobuf::ConformanceTestSuite;
using std::string;
using std::vector;

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

uint32_t run_rust(
    const uint8_t* data,
    uint32_t len,
    uint8_t* odata,
    uint32_t olen);

} // extern "C"


class rust_runner : public google::protobuf::ConformanceTestRunner {
 public:
  virtual void RunTest(const std::string& test_name,
                       const std::string& input,
                       std::string* output) {

      // 0xE1, 0x88, 0xB4
      // 0x12, 0x34
    if (test_name == "Required.Proto3.TextFormatInput.StringLiteralOctalEscapesString.ProtobufOutput") {
      printf("TEST");
    }
    uint8_t data[8192];
    uint32_t out_len =
        run_rust((const uint8_t*) input.data(), input.length(), data, 8192);
    *output = string((char*) data, (size_t) out_len);
  }

  static int Run(int argc, char* argv[],
                 const std::vector<ConformanceTestSuite*>& suites) {
    if (suites.empty()) {
      fprintf(stderr, "No test suites found.\n");
      return EXIT_FAILURE;
    }
    bool all_ok = true;
    for (ConformanceTestSuite* suite: suites) {
      std::vector<string> program_args;
      string failure_list_filename;
      conformance::FailureSet failure_list;
      bool performance = false;
      suite->SetOutputDir(".");
//      suite->SetVerbose(true);

//    for (int arg = 1; arg < argc; ++arg) {
//      if (strcmp(argv[arg], suite->GetFailureListFlagName().c_str()) == 0) {
//        if (++arg == argc) UsageError();
//        failure_list_filename = argv[arg];
//        ParseFailureList(argv[arg], &failure_list);
//      } else if (strcmp(argv[arg], "--performance") == 0) {
//        performance = true;
//        suite->SetPerformance(true);
//      } else if (strcmp(argv[arg], "--verbose") == 0) {
//        suite->SetVerbose(true);
//      } else if (strcmp(argv[arg], "--enforce_recommended") == 0) {
//        suite->SetEnforceRecommended(true);
//      } else if (strcmp(argv[arg], "--output_dir") == 0) {
//        if (++arg == argc) UsageError();
//        suite->SetOutputDir(argv[arg]);
//      } else if (argv[arg][0] == '-') {
//        bool recognized_flag = false;
//        for (ConformanceTestSuite* suite: suites) {
//          if (strcmp(argv[arg], suite->GetFailureListFlagName().c_str()) == 0) {
//            if (++arg == argc) UsageError();
//            recognized_flag = true;
//          }
//        }
//        if (!recognized_flag) {
//          fprintf(stderr, "Unknown option: %s\n", argv[arg]);
//          UsageError();
//        }
//      } else {
//        program += argv[arg++];
//        while (arg < argc) {
//          program_args.push_back(argv[arg]);
//          arg++;
//        }
//      }
//    }

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
