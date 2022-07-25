## Description of cases covered

The following cases have been tested and covered under tests.

    1. case_01
       - genesis -> deposit
       - should succeed
  
    2. case_02
       - genesis -> withdraw
       - ignore
  
    3. case_03
       - genesis -> dispute
       - ignore
   
    4. case_04
       - genesis -> chargeback
       - ignore
   
    5. case_05
       - genesis -> resolve
       - ignore
   
    6. case_06
       - genesis -> deposit -> withdraw
       - succeed
   
    7. case_07
       - genesis -> deposit -> dispute
       - succeed
   
    8. case_08
       - genesis -> deposit -> resolve
       - ignore
   
    9. case_09
       - genesis -> deposit -> chargeback
       - ignore
   
    10. case_10
       - genesis -> deposit -> withdraw -> dispute
       - ignore
   
    11. case_11
       - genesis -> deposit -> dispute -> dispute
       - ignore
   
    12. case_12
       - genesis -> deposit -> dispute -> resolve
       - succeed
   
    13. case_13
       - genesis -> deposit -> dispute -> chargeback
       - succeed
   
    14. case_14
       - genesis -> deposit -> dispute -> withdraw
       - ignore
   
    15. case_15
       - genesis -> deposit -> dispute -> resolve -> dispute
       - ignore
   
    16. case_16
       - genesis -> deposit -> dispute -> resolve -> resolve
       - ignore
   
    17. case_17
       - genesis -> deposit -> dispute -> resolve -> chargeback
       - ignore
   
    18. case_18
       - genesis -> deposit -> dispute -> chargeback -> chargeback
       - ignore
   
    19. case_19
       - genesis -> deposit -> dispute -> chargeback -> deposit
       - ignore
   
    20. case_20
       - genesis -> deposit -> dispute -> chargeback -> withdraw
       - ignore
   
    21. case_21
       - genesis -> deposit -> dispute -> chargeback -> dispute
       - ignore
   
    22. case_22
       - genesis -> deposit -> partial_withdraw -> full_withdraw
       - ignore
