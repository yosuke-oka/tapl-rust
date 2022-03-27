/* Examples for testing */

 lambda x:Bool. x;
 (lambda x:Bool->Bool. if x false then true else false) 
   (lambda x:Bool. if x then false else true); 
 let x=(lambda x:Bool. x) in (lambda x:Bool->Bool. if x false then true else false) 
   (lambda x:Bool. if x then false else true); 
 let x= (lambda x:Bool->Bool. if x false then true else false) 
   (lambda x:Bool. if x then false else true) in (lambda x:Bool->Bool. if x false then true else false) 
   (lambda x:Bool. if x then false else true); 
