#include <jjs/jtl.h>

int main() {
    testgen::TestgenSession args = testgen::init();
    fprintf(args.out_file, "%d %d\n", args.test_id, args.test_id * 2 + 1);
}