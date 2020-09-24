---- MODULE MC ----
EXTENDS sm_vault, TLC

\* MV CONSTANT declarations@modelParameterConstants
CONSTANTS
M1, M2, M3, M4
----

\* MV CONSTANT definitions MASTERS
const_1600935766525334000 == 
{M1, M2, M3, M4}
----

\* SYMMETRY definition
symm_1600935766525335000 == 
Permutations(const_1600935766525334000)
----

\* CONSTANT definitions @modelParameterConstants:0TOTAL_COINS
const_1600935766525336000 == 
7
----

\* CONSTANT definitions @modelParameterConstants:2TOTAL_STEPS
const_1600935766525337000 == 
3
----

\* CONSTANT definitions @modelParameterConstants:3ACCOUNT_COUNT
const_1600935766525338000 == 
4
----

\* Constant expression definition @modelExpressionEval
const_expr_1600935766525339000 == 
ValidCoins
----

\* Constant expression ASSUME statement @modelExpressionEval
ASSUME PrintT(<<"$!@$!@$!@$!@$!",const_expr_1600935766525339000>>)
----

\* INIT definition @modelBehaviorNoSpec:0
init_1600935766525340000 ==
FALSE/\LAYER = 0
----
\* NEXT definition @modelBehaviorNoSpec:0
next_1600935766525341000 ==
FALSE/\LAYER' = LAYER
----
=============================================================================
\* Modification History
\* Created Thu Sep 24 11:22:46 IDT 2020 by yaronwittenstein
