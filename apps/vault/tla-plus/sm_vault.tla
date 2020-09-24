------------------------------ MODULE sm_vault ------------------------------
EXTENDS TLC, Integers, Sequences, FiniteSets

CONSTANTS MASTERS, ACCOUNT_COUNT, TOTAL_COINS, TOTAL_STEPS

ASSUME ACCOUNT_COUNT = 4
ASSUME Cardinality(MASTERS) = ACCOUNT_COUNT

ASSUME TOTAL_COINS \in 0..10
ASSUME TOTAL_STEPS \in 1..10

(* --algorithm sm_vault

variables   
    STEP = 0,
    
    LAYER = 0,
    
    InitCoins = Pick(ValidCoins),
    
    InitMasters = LET
                    M1 == Pick(MASTERS)
                    M2 == Pick(MASTERS \ {M1})
                    M3 == Pick((MASTERS \ {M1}) \ {M2})
                    M4 == Pick(((MASTERS \ {M1}) \ {M2}) \ {M3})
                  IN
                    <<M1, M2, M3, M4>>,
    
    ACCOUNTS =  LET ConsAcc(I) == [
                    Master |-> InitMasters[I], 
                    Balance |-> InitCoins[I]
                ]
                IN [I \in 1..ACCOUNT_COUNT |-> ConsAcc(I)],      
         
    VAULT =  
        LET M1 == Pick(MASTERS)
            M2 == Pick(MASTERS \ {M1})
            M3 == Pick((MASTERS \ {M1}) \ {M2})
        IN
            [
                Masters |-> {M1, M2, M3},
                Balance |-> 0,
                Pending |-> [Master |-> {}, Amount |-> {}]
            ]

define    
    Pick(S) == CHOOSE s \in S: TRUE
    
    PickN(S, N) == CHOOSE s \in SUBSET S: Cardinality(s) = N
    
    Range(f) == {f[x] : x \in DOMAIN f}
    
    RECURSIVE SeqSum(_)
    SeqSum(S) ==
        IF S = <<>> THEN
            0
        ELSE
            Head(S) + SeqSum(Tail(S))
                                 
    ValidCoins == 
        LET All == [1..ACCOUNT_COUNT -> 0..TOTAL_COINS]
        IN 
           {C \in All: SeqSum(C) = TOTAL_COINS}
    
    
    
    VaultIsPending == VAULT.Pending.Master /= {}
    
    VaultPendingMaster == VAULT.Pending.Master
    
    VaultPendingAmount == VAULT.Pending.Amount
    
    AccountByMaster(M) == CHOOSE ACC \in Range(ACCOUNTS): ACC.Master = M
          
    NonNegeBalanceInvariant == (\A I \in 1..ACCOUNT_COUNT: ACCOUNTS[1].Balance >= 0)
                                    /\
                                (VAULT.Balance >= 0)
           
    TotalCoinsInvariant == SeqSum([I \in 1..ACCOUNT_COUNT |-> ACCOUNTS[I].Balance]) 
                           + VAULT.Balance 
                         = TOTAL_COINS                        
end define;
    

macro FundVault(FROM, AMOUNT)
begin
    assert FROM.Balance >= AMOUNT;
    
    FROM.Balance := FROM.Balance - AMOUNT;
    VAULT.Balance := VAULT.Balance + AMOUNT;
end macro;

macro FundAccount(ACC, AMOUNT) 
begin
    ACC.Balance := ACC.Balance + AMOUNT
end macro;

macro WithdrawBegin(ACC, AMOUNT) 
begin
    assert ~VaultIsPending;
    assert VAULT.Balance >= AMOUNT;
    
    VAULT.Pending := [Master |-> ACC.Master, Amount |-> AMOUNT]
end macro;

macro WithdrawApprove(ACC)
begin
    assert VaultIsPending;
    assert VAULT.Balance >= VaultPendingAmount;
    
    if (ACC.Master \in VAULT.Masters) /\ (ACC.Master /= VaultPendingMaster) then
        VAULT.Balance := VAULT.Balance - VaultPendingAmount;
        
        \* FundAccount(AccountByMaster(VaultPendingMaster), VaultPendingAmount);
        \* VAULT.Pending := [Master |-> {}, Amount |-> {}];
    end if;  
end macro;

begin  
    print ACCOUNTS;

    while STEP < TOTAL_STEPS do     
        \* we always increment `STEP`
        STEP := STEP + 1;
        
        either 
            \* Withdraw (Begin / Approve)
            
            if ~VaultIsPending then
                \* Withdraw begin
                
                with I \in 1..ACCOUNT_COUNT,
                    AMOUNT \in 0..VAULT.Balance
                do
                    WithdrawBegin(ACCOUNTS[I], AMOUNT)
                end with;
                
            else
                \* Withdraw approve
                 
                 with M \in VAULT.Masters do
                    skip;
                    \* WithdrawApprove(AccountByMaster(M))
                 end with;
            end if; 
        or
            \* Fund Vault
            with I \in 1..ACCOUNT_COUNT,
                AMOUNT \in 0..ACCOUNTS[I].Balance
            do
                FundVault(ACCOUNTS[I], AMOUNT)
            end with; 
        or
            \* Next layer    
            LAYER := LAYER + 1  
        end either;
    end while;  
    
end algorithm; *)

\* BEGIN TRANSLATION - the hash of the PCal code: PCal-4e467e5a0b5a4e30c0bb7c769c002c51
VARIABLES STEP, LAYER, InitCoins, InitMasters, ACCOUNTS, VAULT, pc

(* define statement *)
Pick(S) == CHOOSE s \in S: TRUE

PickN(S, N) == CHOOSE s \in SUBSET S: Cardinality(s) = N

Range(f) == {f[x] : x \in DOMAIN f}

RECURSIVE SeqSum(_)
SeqSum(S) ==
    IF S = <<>> THEN
        0
    ELSE
        Head(S) + SeqSum(Tail(S))

ValidCoins ==
    LET All == [1..ACCOUNT_COUNT -> 0..TOTAL_COINS]
    IN
       {C \in All: SeqSum(C) = TOTAL_COINS}



VaultIsPending == VAULT.Pending.Master /= {}

VaultPendingMaster == VAULT.Pending.Master

VaultPendingAmount == VAULT.Pending.Amount

AccountByMaster(M) == CHOOSE ACC \in Range(ACCOUNTS): ACC.Master = M

NonNegeBalanceInvariant == (\A I \in 1..ACCOUNT_COUNT: ACCOUNTS[1].Balance >= 0)
                                /\
                            (VAULT.Balance >= 0)

TotalCoinsInvariant == SeqSum([I \in 1..ACCOUNT_COUNT |-> ACCOUNTS[I].Balance])
                       + VAULT.Balance
                     = TOTAL_COINS


vars == << STEP, LAYER, InitCoins, InitMasters, ACCOUNTS, VAULT, pc >>

Init == (* Global variables *)
        /\ STEP = 0
        /\ LAYER = 0
        /\ InitCoins = Pick(ValidCoins)
        /\ InitMasters = LET
                           M1 == Pick(MASTERS)
                           M2 == Pick(MASTERS \ {M1})
                           M3 == Pick((MASTERS \ {M1}) \ {M2})
                           M4 == Pick(((MASTERS \ {M1}) \ {M2}) \ {M3})
                         IN
                           <<M1, M2, M3, M4>>
        /\ ACCOUNTS = (LET ConsAcc(I) == [
                           Master |-> InitMasters[I],
                           Balance |-> InitCoins[I]
                       ]
                       IN [I \in 1..ACCOUNT_COUNT |-> ConsAcc(I)])
        /\ VAULT = (LET M1 == Pick(MASTERS)
                        M2 == Pick(MASTERS \ {M1})
                        M3 == Pick((MASTERS \ {M1}) \ {M2})
                    IN
                        [
                            Masters |-> {M1, M2, M3},
                            Balance |-> 0,
                            Pending |-> [Master |-> {}, Amount |-> {}]
                        ])
        /\ pc = "Lbl_1"

Lbl_1 == /\ pc = "Lbl_1"
         /\ PrintT(ACCOUNTS)
         /\ pc' = "Lbl_2"
         /\ UNCHANGED << STEP, LAYER, InitCoins, InitMasters, ACCOUNTS, VAULT >>

Lbl_2 == /\ pc = "Lbl_2"
         /\ IF STEP < TOTAL_STEPS
               THEN /\ STEP' = STEP + 1
                    /\ \/ /\ IF ~VaultIsPending
                                THEN /\ \E I \in 1..ACCOUNT_COUNT:
                                          \E AMOUNT \in 0..VAULT.Balance:
                                            /\ Assert(~VaultIsPending, 
                                                      "Failure of assertion at line 100, column 5 of macro called at line 135, column 21.")
                                            /\ Assert(VAULT.Balance >= AMOUNT, 
                                                      "Failure of assertion at line 101, column 5 of macro called at line 135, column 21.")
                                            /\ VAULT' = [VAULT EXCEPT !.Pending = [Master |-> (ACCOUNTS[I]).Master, Amount |-> AMOUNT]]
                                ELSE /\ \E M \in VAULT.Masters:
                                          TRUE
                                     /\ VAULT' = VAULT
                          /\ UNCHANGED <<LAYER, ACCOUNTS>>
                       \/ /\ \E I \in 1..ACCOUNT_COUNT:
                               \E AMOUNT \in 0..ACCOUNTS[I].Balance:
                                 /\ Assert((ACCOUNTS[I]).Balance >= AMOUNT, 
                                           "Failure of assertion at line 87, column 5 of macro called at line 151, column 17.")
                                 /\ ACCOUNTS' = [ACCOUNTS EXCEPT ![I].Balance = (ACCOUNTS[I]).Balance - AMOUNT]
                                 /\ VAULT' = [VAULT EXCEPT !.Balance = VAULT.Balance + AMOUNT]
                          /\ LAYER' = LAYER
                       \/ /\ LAYER' = LAYER + 1
                          /\ UNCHANGED <<ACCOUNTS, VAULT>>
                    /\ pc' = "Lbl_2"
               ELSE /\ pc' = "Done"
                    /\ UNCHANGED << STEP, LAYER, ACCOUNTS, VAULT >>
         /\ UNCHANGED << InitCoins, InitMasters >>

(* Allow infinite stuttering to prevent deadlock on termination. *)
Terminating == pc = "Done" /\ UNCHANGED vars

Next == Lbl_1 \/ Lbl_2
           \/ Terminating

Spec == Init /\ [][Next]_vars

Termination == <>(pc = "Done")

\* END TRANSLATION - the hash of the generated TLA code (remove to silence divergence warnings): TLA-f33ae4006082a625cb88018c0d17dd74


=============================================================================
\* Modification History
\* Last modified Thu Sep 24 11:22:05 IDT 2020 by yaronwittenstein
\* Created Wed Sep 23 10:52:51 IDT 2020 by yaronwittenstein
