package main

/*
#cgo LDFLAGS: -L../rust/target/release -lmultisig
#include <stdint.h>

void gen_keypair(
    uint64_t seed,
    uint32_t activation_epoch
);

void message_signing(
    uint64_t seed,
    uint32_t activation_epoch,
    uint32_t signing_epoch
);

void sig_verification(
    uint64_t seed,
    uint32_t activation_epoch,
    uint32_t signing_epoch
);
*/
import "C"

import "fmt"

func main() {
    C.gen_keypair(
        C.uint64_t(1),
        C.uint32_t(5),
    )
    
    C.message_signing(
        C.uint64_t(1),
        C.uint32_t(5),
        C.uint32_t(4),
    )
    
    C.sig_verification(
        C.uint64_t(1),
        C.uint32_t(5),
        C.uint32_t(4),
    )

    fmt.Println("gen_keypair called successfully")
}
