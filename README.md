### NOTES
- This does not take into account the using queues and there are no concurrent users, however state is managed through redis. (To start this, start a local redis or enter a hosted Redis url)
- BUY Price and SELL Price can be an `Option<f64>` and then the auto price determination logic would come
- Storing values in f64 decimals might have precision issues. We can store them in a very large quantity probably? ( Like SOLANA does with lamports )   

## EQUAL BUY SELL QUANTITY

https://github.com/user-attachments/assets/9d16d542-b843-4153-afae-3b642897a6ab

## QUANTITY OF THE BUY ORDER IS LESS THAN THE QUANTITY OF SELL ORDER IN A SPECIFIC CLUSTER

https://github.com/user-attachments/assets/31a5e0c0-173f-4430-b9aa-7716da9d3625

