https://solidity-by-example.org/


Need a way to recreate the contract from its stored data 

Think I'll need to move away from setting id in constructor for StorageItem?
Can I do like solidity, and use instead as id the number in storage?
like first item is number 1, second is 2...
Could be doable with Struct as second argument of generics

like
``` 
struct First {}
```

or use macro to modify it? 

do I even need all this? 
Context should be able to handle it maybe?
no, context doesn't have access to App, so can't? 


WAIT UP
Can't I just clone the contract on instantiation, and force reload the state? 
I still need to get access to the state from the app though 

or maybe, can I store it using a generic?
something like 
store<T>(target, serialized_object) ?

wait up, can't i maybe deserialize it from the debug representation? 
like 
struct SampleContract {val: u64}

repr should be 
SampleContract {val: 8} ?

can also derive the from state loader from macro, from constructor
issue with what i had: difference between a string and a litteral 


maybe from this 
https://users.rust-lang.org/t/using-a-string-as-a-type-name-in-macros-solved/11988/4

maybe can I generate a new file using a build.rs?