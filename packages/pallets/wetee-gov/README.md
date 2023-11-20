# ***Gov Module***
***

***
## All Calls
***
### For every call
* `set_min_vote_weight_for_every_call` Set origin for a specific call.
### For some Storage
* `set_max_public_props` Set the maximum number of proposals at the same time.
* `set_launch_period` Set the referendum interval.
* `set_minimum_deposit` Set the minimum amount a proposal needs to stake.
* `set_voting_period` Set the voting length of the referendum.
* `set_rerserve_period` Set the length of time that can be unreserved.
* `set_enactment_period` Set the time to delay the execution of the proposal.

### For Voting
* `propose` Initiate a proposal.
* `recreate` Support recreate proposals.
* `deposit_proposal` Open a referendum.
* `vote_for_referendum` Vote for the referendum.
* `cancel_vote` Cancel a vote on a referendum.
* `run_proposal` Vote and execute the transaction corresponding to the proposa.
* `unlock` Release the locked amount.
