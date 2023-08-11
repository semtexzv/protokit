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
//    if (test_name == "Required.Proto2.ProtobufInput.ValidDataOneof.UINT32.MultipleValuesForDifferentField.ProtobufOutput") {
//       printf("TEST");
//    }
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
