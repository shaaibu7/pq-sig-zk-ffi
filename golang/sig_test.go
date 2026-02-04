// multisig_bench_test.go
package main

import "testing"

func BenchmarkGenKeypair(b *testing.B) {
	GenKeypair() // warm-up
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		GenKeypair()
	}
}

func BenchmarkMessageSigning(b *testing.B) {
	for i := 0; i < b.N; i++ {
		MessageSigning()
	}
}

func BenchmarkSigVerification(b *testing.B) {
	for i := 0; i < b.N; i++ {
		SigVerification()
	}
}

func BenchmarkNoop(b *testing.B) {
	for i := 0; i < b.N; i++ {
		Noop()
	}
}
