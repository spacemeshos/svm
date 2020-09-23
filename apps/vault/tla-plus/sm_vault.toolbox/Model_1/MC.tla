---- MODULE MC ----
EXTENDS sm_vault, TLC

\* MV CONSTANT declarations@modelParameterConstants
CONSTANTS
M1, M2, M3, M4
----

\* MV CONSTANT definitions MASTERS
const_1600888446665163000 == 
{M1, M2, M3, M4}
----

\* SYMMETRY definition
symm_1600888446665164000 == 
Permutations(const_1600888446665163000)
----

\* CONSTANT definitions @modelParameterConstants:0TOTAL_COINS
const_1600888446665165000 == 
7
----

\* CONSTANT definitions @modelParameterConstants:2TOTAL_STEPS
const_1600888446665166000 == 
3
----

\* CONSTANT definitions @modelParameterConstants:3ACCOUNT_COUNT
const_1600888446665167000 == 
4
----

=============================================================================
\* Modification History
\* Created Wed Sep 23 22:14:06 IDT 2020 by yaronwittenstein
