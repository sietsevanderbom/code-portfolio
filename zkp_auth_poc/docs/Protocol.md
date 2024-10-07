# Chaum–Pedersen Protocol

We now present a Sigma protocol called the Chaum–Pedersen protocol which was first presented in the context of electronic cash systems, but which has very wide application.

Suppose Peggy wishes to prove she knows two discrete logarithms

y1=g^x1 and y2=h^x2

such that x1=x2, i.e. we wish to present both a proof of knowledge of the discrete logarithm, but also a proof of equality of the hidden discrete logarithms. We assume that g and h generate groups of prime order q, and we denote the common discrete logarithm by x ease notation. Using our prior notation for Sigma protocols, the Chaum–Pedersen protocol can be expressed via:

R(x,k) = (r1,r2) = (g^k, h^k),
S(c,x,k) = s = k − c * x (mod q),
V((r1,r2),c,s) = true <=> (r1 = g^s * y1^c and r2 = h^s * y2^c),
S'(c,s) = (r1,r2) = (g^s * y1^c, h^s * y2^c).

Note, how this resembles two concurrent runs of the Schnorr protocol.
The Chaum–Pedersen protocol is clearly both complete and has the zero-knowledge property,
the second fact follows since the simulation S′(c,s) produces transcripts which are is indistin- guishable from a real transcript. We need to show it is sound, however since we are assuming honest-verifiers we only need to show it has the special-soundness property. Hence, we assume two protocol runs with the same commitments (t1, t2), different challenges, c1 and c2 and valid responses s1 and s2. With this data we need to show that this reveals the common discrete logarithm. Since the two transcripts pass the verification test we have such that

t1 = g^s1 * y1^c1 = g^s2 * y1^c2 and t2 = h^s1 * y2^c1 = h^s2 * y2^c2

But this implies that:

y1^(c1-c2) = g^(s2-s1) and y2^(c2-c1) = h^(s2-s1)

Hence, the two discrete logarithms are equal and can be extracted from:

x = ((c1 − c2)/ (s2 − s1))  (mod q).
