# .mk files
MK += Makefile
MK += $(wildcard   mk/*.mk)
MK += $(wildcard   hw/*.mk)
MK += $(wildcard  cpu/*.mk)
MK += $(wildcard arch/*.mk)
MK += $(wildcard   os/*.mk)

# ini
S += $(wildcard lib/*.ini) $(wildcard lib/*.f)

# Rust
R += $(wildcard      ./src/*.rs)      ./Cargo.toml
