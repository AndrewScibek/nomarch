# Changes
  
* Config
  * change max_seconds_to_reach_end to expected_seconds_to_reach_end
  * add oldest_event_to_track related to above
* Logs
  * Add # of events older than expected_seconds_to_reach_end in tick log
  * can log them differently(new state). could prob do some cool agregates
* Processor
  * would need to inspect more events more often
  * events would be inspected quicker to have better insight

Reasoning: latency of getting the request to nomarch is not always a priority or even possible in some cases. By knowing that 99.999% of events are flowing fine and its probably just a few events that should finish soon. I think this change would help prevent setting wildly high max_seconds that isnt actually what they are trying to track/should actually be how long it takes.
