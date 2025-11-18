
<div align="center">

# Rust BF Runtime Interface (RBFRT)

 ![image](https://img.shields.io/badge/licence-Apache%202.0-blue)
![Build with badge](https://img.shields.io/badge/Build_with-Rust-red)
![image](https://img.shields.io/badge/v-0.1.8alpha-yellow)
[![build](https://github.com/uni-tue-kn/rbfrt/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/uni-tue-kn/rbfrt/actions/workflows/build.yml)

</div>

- [Rust BF Runtime Interface (RBFRT)](#rust-bf-runtime-interface-rbfrt)
  - [Overview](#overview)
  - [Documentation](#documentation)
  - [Cite](#cite)


## Overview

This repository contains the source code of *Rust Barefoot Runtime (RBFRT): Fast Runtime Control for the Intel Tofino* ([preprint paper](https://arxiv.org/abs/2501.17271)).

The RBFRT is a Rust-based control plane library.
It provides a fast and memory-safe interface to configure the Intel Tofino.
It uses gRPC and the official Protobuf definitions of the [Open-Tofino](https://github.com/barefootnetworks/Open-Tofino) GitHub repository.

## Documentation

The documentation of this crate is deployed as a [GitHub page](https://uni-tue-kn.github.io/rbfrt/rbfrt/).

## Cite
If you use RBFRT in any of your publications, please cite the following papers:
1. E. Zink, M. Flüchter, S. Lindner, F. Ihle, and M. Menth: [Rust Barefoot Runtime (RBFRT): Fast Runtime Control for the Intel Tofino](https://publikationen.uni-tuebingen.de/xmlui/bitstream/handle/10900/163778/4th_kuvs_fg_netsoft_13.pdf), in KuVS Workshop on Network Softwarization (KuVS NetSoft), Apr. 2025, online

```tex
@article{ZiFl25,
  title  = {{Rust Barefoot Runtime (RBFRT): Fast Runtime Control for the Intel Tofino}},
  author = {Etienne Zink and Moritz Flüchter and Steffen Lindner and Fabian Ihle and Michael Menth},
  journal = {{KuVS Workshop on Network Softwarization (KuVS NetSoft)}}
  year   = 2025,
  month  = apr
}
```
