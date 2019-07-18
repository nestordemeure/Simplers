use priority_queue::PriorityQueue;
use ordered_float::OrderedFloat;
use crate::simplex::*;

//-----------------------------------------------------------------------------
// QUEUE

/// a queue that uses an index to identify the simplex instead of a Hash
pub struct Queue
{
   queue: PriorityQueue<Simplex, OrderedFloat<f64>>,
   exploration_preference: f64,
   previous_difference: f64
}

impl Queue
{
   pub fn new(exploration_preference: f64) -> Queue
   {
      let queue = PriorityQueue::new();
      let previous_difference = 0.;
      Queue { queue, exploration_preference, previous_difference }
   }

   pub fn push(&mut self, key: Simplex, difference: f64)
   {
      let value = key.evaluate(self.exploration_preference, difference);
      self.queue.push(key, OrderedFloat(value));
   }

   pub fn pop(&mut self) -> Simplex
   {
      let (simplex, _value) = self.queue.pop().expect("The queue is empty!");
      simplex
   }
}
