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
    
    23. case_23
       - deposit negative amount
       - ignore
  
    24. case_24
       - withdraw negative amount
       - ignore
    
    25. case_25
       - deposit amount that is max allowed by container type
       - deposit amount that max allowed by container type
       - ignore second deposit
    
    26. case_26
       - deposit amount larger than max allowed by container type
       - ignore

    27. case_27
       - deposit without amount
       - ignore / no impact on funds

    28. case_28
       - withdraw without amount
       - ignore / no impact on funds

    29. case_29
       - deposit without client id
       - ignore

    30. case_30
       - deposit tx id
       - ignore

    31. case_31
       - withdraw without client id
       - ignore

    32. case_32
       - withdraw without tx id
       - ignore

    33. case_33
       - dispute without client id
       - ignore

    34. case_34
       - dispute without tx id
       - ignore

    35. case_35
       - resolve without client id
       - ignore

    36. case_36
       - resolve without tx id
       - ignore

    37. case_37
       - chargeback without client id
       - ignore

    38. case_38
       - chargeback without tx id
       - ignore

    39. case_39
       - withdraw amount exactly f64::MAX
       - ignore

    40. case_40
       - try negative number as client id
       - ignore

    41. case_41
       - try client id larger than u16
       - ignore

    42. case_42
       - try negative number as tx id
       - ignore

    43. case_43
       - try tx id larger than u32 
       - ignore
