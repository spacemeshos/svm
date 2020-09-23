---- MODULE MC ----
EXTENDS sm_vault, TLC

\* MV CONSTANT declarations@modelParameterConstants
CONSTANTS
M1, M2, M3, M4
----

\* MV CONSTANT definitions MASTERS
const_1600874172008460000 == 
{M1, M2, M3, M4}
----

\* SYMMETRY definition
symm_1600874172008461000 == 
Permutations(const_1600874172008460000)
----

\* CONSTANT definitions @modelParameterConstants:0TOTAL_COINS
const_1600874172008462000 == 
7
----

\* CONSTANT definitions @modelParameterConstants:2TOTAL_STEPS
const_1600874172008463000 == 
8
----

\* CONSTANT definitions @modelParameterConstants:3ACCOUNT_COUNT
const_1600874172008464000 == 
4
----

=============================================================================
\* Modification History
\* Created Wed Sep 23 18:16:12 IDT 2020 by yaronwittenstein
