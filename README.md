# Rust BF Runtime Interface (RBFRT)

![Build with badge](https://img.shields.io/badge/Build_with-Rust-red)
[![build](https://github.com/uni-tue-kn/rbfrt/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/uni-tue-kn/rbfrt/actions/workflows/rust.yml)

- [Rust BF Runtime Interface (RBFRT)](#rust-bf-runtime-interface-rbfrt)
  - [Overview](#overview)
  - [Documentation](#documentation)

## Overview

This repository contains the source code of *Rust Barefoot Runtime (RBFRT): Fast Runtime Control for the Intel Tofino* ([preprint paper](https://arxiv.org/abs/2501.17271)).

The RBFRT is a Rust-based control plane library.
It provides a fast and memory-safe interface to configure the Intel Tofino.
It uses gRPC and the official Protobuf definitions of the [Open-Tofino](https://github.com/barefootnetworks/Open-Tofino) GitHub repository.

## Documentation

The documentation of this crate is deployed as a [GitHub page](https://uni-tue-kn.github.io/rbfrt/rbfrt/).
