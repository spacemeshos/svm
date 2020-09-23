------------------------------ MODULE sm_vault ------------------------------
EXTENDS TLC, Integers, Sequences, FiniteSets

CONSTANTS MASTERS, TOTAL_COINS

ASSUME Len(MASTERS) = 3
ASSUME TOTAL_COINS \in 0..10

(* --algorithm sm_vault

variables
 
    
    
define
    Pick(S) == CHOOSE s \in S: TRUE
    
    RECURSIVE SUM(_)
    SUM(S) == IF S = {} THEN
                0
             ELSE
                LET s == Pick(S)
                IN s + SUM(S \ {s})
                
  
                                        
    AccountsCoins ==
        LET All == {{x, y, z}: x \in 0..TOTAL_COINS, y \in 0..TOTAL_COINS, z \in 0..TOTAL_COINS}
            Valid == {C \in All: SUM(C) = TOTAL_COINS}
        IN 
           {Seq(S): S \in Valid}

end define;

    

begin
    skip;

end algorithm; *)

\* BEGIN TRANSLATION - the hash of the PCal code: PCal-db8c986841ae5679968c784b94af23b7
VARIABLE pc

(* define statement *)
Pick(S) == CHOOSE s \in S: TRUE

RECURSIVE SUM(_)
SUM(S) == IF S = {} THEN
            0
         ELSE
            LET s == Pick(S)
            IN s + SUM(S \ {s})



AccountsCoins ==
    LET All == {{x, y, z}: x \in 0..TOTAL_COINS, y \in 0..TOTAL_COINS, z \in 0..TOTAL_COINS}
        Valid == {C \in All: SUM(C) = TOTAL_COINS}
    IN
       {Seq(S): S \in Valid}


vars == << pc >>

Init == /\ pc = "Lbl_1"

Lbl_1 == /\ pc = "Lbl_1"
         /\ TRUE
         /\ pc' = "Done"

(* Allow infinite stuttering to prevent deadlock on termination. *)
Terminating == pc = "Done" /\ UNCHANGED vars

Next == Lbl_1
           \/ Terminating

Spec == Init /\ [][Next]_vars

Termination == <>(pc = "Done")

\* END TRANSLATION - the hash of the generated TLA code (remove to silence divergence warnings): TLA-12b8cbbe288e88f1f2e8d5e5a166bd73


=============================================================================
\* Modification History
\* Last modified Wed Sep 23 11:44:29 IDT 2020 by yaronwittenstein
\* Created Wed Sep 23 10:52:51 IDT 2020 by yaronwittenstein
