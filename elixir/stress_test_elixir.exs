#   DELETE THIS MODULE WHEN TESTING  #
#  Call your own Poker.deal function #

defmodule Poker do
    def deal _perm do
        # Dummy module for the stress tester to call. Your own module
        # with its deal function should be defined and compiled separately.
        ["2S", "3S", "4S", "5S", "6S"]   # CORRECT
        #[2, "3S", "4S", "5S", "6S"]     # elems not strings
        #["2S", "3S", 4, "5S", "6S"]     # elems not strings
        #["2S", "3S", "4S", "5S"]        # length != 5
        #"2S, 3S, 4S, 5S, 6S"            # output not a list
    end
end

nTrials = 100 # Make this larger for more tests
trials = 1..nTrials
deck = 1..52 |> Enum.to_list()

res = for _n <- trials do

    perm = deck |> Enum.shuffle |> Enum.slice(0, 9)
    IO.write "Testing input: "
    IO.inspect(perm, charlists: :aslists)

    try do
        winner = Poker.deal perm
        IO.write "You returned:  "
        IO.inspect(winner, charlists: :aslists)
        cond do
            not is_list(winner) ->
                IO.puts "FAILED: Not a list"; false
            (length winner) != 5 ->
                IO.puts "FAILED: Length != 5"; false
            not Enum.all?(winner, fn c -> is_bitstring(c) end) ->
                IO.puts "FAILED: Elems not strings"; false
            true ->
                IO.puts "SUCCESS: Output is valid"; true
        end
    rescue
        MatchError ->
            IO.puts "FAILED: MatchError"; false
        Protocol.UndefinedError ->
            IO.puts "FAILED: UndefinedError"; false
        CondClauseError ->
            IO.puts "FAILED: CondClauseError"; false
        UndefinedFunctionError ->
           IO.puts "FAILED: UndefinedFunctionError"; false
        FunctionClauseError ->
            IO.puts "FAILED: FunctionClauseError"; false
    end
end

IO.puts "\nSUCCEEDED #{length(Enum.filter(res, &(&1)))}/#{nTrials} random deals\n"
IO.puts "SUCCESS DOES NOT MEAN YOUR WINNER IS CORRECT. IT MEANS YOUR OUTPUT IS"
IO.puts "PROPERLY FORMATTED/STRUCTURED AS A LIST OF FIVE STRINGS.\n"
