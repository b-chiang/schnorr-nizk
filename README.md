# schnorr-nizk

An implementation of a non-interactive proof of knowledge of discrete log. Specifically, Schnorr's protocol (interactive, PoK of discete log) is made non-interactive and fully ZK through the Fiat-Shamir transform.

Given a group $G$ of prime-order $q$ and generator $g$, Alice wants to prove to Bob that she knows $x$ such that $u = g^x$, where both parties know $u$. For more details on the protocol, please see [these notes](https://crypto.stanford.edu/cs355/19sp/lec11.pdf). [Ristretto](https://ristretto.group/) was used for the prime-order group.
