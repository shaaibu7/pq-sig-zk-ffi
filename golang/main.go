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

void noop();

void sig_verification(
    uint64_t seed,
    uint32_t activation_epoch,
    uint32_t signing_epoch
);
*/
import "C"

func GenKeypair() {
	C.gen_keypair(C.uint64_t(1), C.uint32_t(5))
}

func MessageSigning() {
	C.message_signing(C.uint64_t(1), C.uint32_t(5), C.uint32_t(4))
}

func SigVerification() {
	C.sig_verification(C.uint64_t(1), C.uint32_t(5), C.uint32_t(4))
}

func Noop() {
	C.noop()
}

func main() {
	GenKeypair()
	MessageSigning()
	SigVerification()
}
