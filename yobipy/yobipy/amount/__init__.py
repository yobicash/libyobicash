class YAmount:
   """ Represents a yobicash amount
    
   Keyword args:
      amount: if not None, initialise the amount from a YBigUint, python integer or python string

   """
   def __init__(self,amount=None):
       if type(amount) is str:
          pass # TODO - parse into YBigUint here
       elif type(amount) is int:
          pass # TODO - turn python int into YBigUint here
       else: # assume that it's a YBigUint instance already
          pass
   def __repr__(self):
       pass # TODO - map this to the to_string() rust method
   def __str__(self):
       pass # TODO - map this to the Display rust method
   def __int__(self):
       pass # TODO - map this to the to_u64() rust method
   def __bytes__(self):
       pass # TODO - map this to to_bytes() rust method
