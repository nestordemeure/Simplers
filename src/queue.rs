use priority_queue::PriorityQueue;
use std::collections::HashMap;
use ordered_float::OrderedFloat;
use crate::simplex::*;

//-----------------------------------------------------------------------------
// QUEUE

pub struct Queue
{
   queue: PriorityQueue<usize, OrderedFloat<f64>>,
   simplexes_index: HashMap<usize, Simplex>,
   exploration_preference: f64,
   previous_difference: f64,
   simplex_number: usize
}

impl Queue
{
   pub fn new(exploration_preference: f64) -> Queue
   {
      let queue = PriorityQueue::new();
      let simplexes_index = HashMap::new();
      let previous_difference = 0.;
      let simplex_number = 0;
      Queue { queue, simplexes_index, exploration_preference, previous_difference, simplex_number }
   }

   pub fn push(&mut self, key: Simplex, difference: f64)
   {
      // updates all the priorities if the difference has changed
      // TODO we could update on pop but it require storing the preference for each simplex in memory
      if difference != self.previous_difference
      {
         self.previous_difference = difference;
         for (id, simplex) in self.simplexes_index.iter()
         {
            let value = simplex.evaluate(self.exploration_preference, difference);
            self.queue.change_priority(&id, OrderedFloat(value));
         }
      }

      // inserts the new element
      let value = key.evaluate(self.exploration_preference, difference);
      let id = self.simplex_number;
      self.simplexes_index.insert(id, key);
      self.queue.push(id, OrderedFloat(value));
      self.simplex_number += 1;
   }

   pub fn pop(&mut self) -> Simplex
   {
      let (id, _value) = self.queue.pop().expect("The queue is empty!");
      self.simplexes_index.remove(&id).expect("Tried to remove a simplex that did not exist!")
   }
}