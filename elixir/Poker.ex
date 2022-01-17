# Author: Adeel Ahmed


defmodule Hand do
	defstruct [cards: [], ranks: %{}, suits: %{}, rank: 0, best_hand: [], straight: 0, string: []]
end


defmodule Poker do
	@suits %{1 => "C",2 => "D",3 => "H",4 => "S"}

	def deal(input) do
		{hand1, hand2} = input |> split

		firstPlayer = %Hand{cards: hand1, ranks: count_ranks(hand1), suits: count_suits(hand1)}
		secondPlayer = %Hand{cards: hand2, ranks: count_ranks(hand2), suits: count_suits(hand2)}

		firstPlayer = hand_check(firstPlayer)
		secondPlayer = hand_check(secondPlayer)

		firstPlayer = %{firstPlayer | string: Enum.map(firstPlayer.best_hand, &num_to_string/1)}
		secondPlayer = %{secondPlayer | string: Enum.map(secondPlayer.best_hand, &num_to_string/1)}

		cond do
			firstPlayer.rank < secondPlayer.rank ->
				firstPlayer.best_hand |> Enum.map(&num_to_string/1) |> Enum.sort
			firstPlayer.rank > secondPlayer.rank ->
				secondPlayer.best_hand |> Enum.map(&num_to_string/1) |> Enum.sort
			firstPlayer.rank == secondPlayer.rank ->
				tie_break(firstPlayer, secondPlayer) |> Enum.map(&num_to_string/1) |> Enum.sort
		end
	end

	# split list of cards into two lists
	def split(cards) do
		first = [elem(Enum.fetch(cards, 0),1)] ++ [elem(Enum.fetch(cards, 2),1)] ++ Enum.take(cards, -5)
		second = [elem(Enum.fetch(cards, 1),1)] ++ [elem(Enum.fetch(cards, 3),1)] ++ Enum.take(cards, -5)
		{first, second}
	end

	# determine which rank each hand belongs to
	def hand_check(hand) do
		cond do
			straight_flush(hand) ->
				%{hand | rank: 2, best_hand: get_straight_flush(hand)}
			four_of_a_kind(hand) ->
				%{hand | rank: 3, best_hand: get_four_of_a_kind(hand)}
			full_house(hand) ->
				%{hand | rank: 4, best_hand: get_full_house(hand)}
			flush(hand) ->
				%{hand | rank: 5, best_hand: get_flush(hand)}
			elem(straight(hand), 0) ->
				%{hand | rank: 6, best_hand: get_straight([], elem(straight(hand), 1), hand)}
			is_three_of_a_kind(hand) ->
				%{hand | rank: 7, best_hand: get_three_of_a_kind(hand)}
			is_two_pairs(hand) ->
				%{hand | rank: 8, best_hand: get_two_pairs(hand)}
			is_pair(hand) ->
				%{hand | rank: 9, best_hand: get_pair(hand)}
			true ->
				%{hand | rank: 10, best_hand: high_card(hand)}
		end
	end

	def tie_break(p1, p2) do
		winner = tie_break_helper(p1.best_hand, p2.best_hand)
		if winner == 1 do
			p1.best_hand
		else
			p2.best_hand
		end
	end

	# 1 for p1, 2 for p2
	def tie_break_helper([h1 | t1], [h2 | t2]) do
		h1 = get_rank(h1)
		h2 = get_rank(h2)
		if h1 == 1 or h2 == 1 do
			if h1 == h2 do
				tie_break_helper(t1, t2)
			else
				if h1 > h2 do
					2
				else
					1
				end
			end
		else
			if h1 == h2 do
				tie_break_helper(t1, t2)
			else
				if h1 > h2 do
					1
				else
					2
				end
			end
		end
	end

	def straight_flush(hand) do
		suit =
			Map.keys(hand.suits)
			|> Enum.max_by(fn n -> hand.suits[n] end)
		if hand.suits[suit] >= 5 do
			filtered = Enum.filter(hand.cards, fn n -> (div(n-1, 13) + 1) == suit end)
			clone = %Hand{cards: filtered, ranks: Poker.count_ranks(filtered), suits: Poker.count_ranks(filtered)}
			elem(straight(clone), 0)
		end
	end

	def get_straight_flush(hand) do
		suit =
			Map.keys(hand.suits)
			|> Enum.max_by(fn n -> hand.suits[n] end)
		filtered = Enum.filter(hand.cards, fn n -> (div(n-1, 13) + 1) == suit end)
		clone = %Hand{cards: filtered, ranks: Poker.count_ranks(filtered), suits: Poker.count_ranks(filtered)}
		straight(clone)
		get_straight([], elem(straight(clone), 1), clone)
	end

	def four_of_a_kind(hand) do
		4 in Map.values(hand.ranks)
	end

	def get_four_of_a_kind(hand) do
		[h | _] = find_keys(hand.ranks, 4)
		fours =	Enum.filter(hand.cards, fn n -> get_rank(n) == h end)
		ones  = Enum.filter(hand.cards, fn n -> get_rank(n) == 1 and get_rank(n) != h end)
		remaining =
			Enum.filter(hand.cards, fn n -> get_rank(n) != h and get_rank(n) != 1 end)
			|> Enum.sort_by(fn n -> get_rank(n) end)
			|> Enum.reverse()
		last_card = Enum.take(ones ++ remaining, 1)
		fours ++ last_card
	end

	# could consist of two trips which is why
	# the size of the map is checked to be less than 5
	# (3-2-1-1) or (3-3-1) (3-2-2)
	def full_house(hand) do
		ranks = Map.values(hand.ranks)
		3 in ranks and 5 > length(ranks)
	end

	def get_full_house(hand) do
		[h | t] = find_keys(hand.ranks, 3)
		Enum.filter(hand.cards, fn n -> get_rank(n) == h end) ++
		if length(t) == 1 do
			[h2 | _] = t
			[h3 | t3] = Enum.filter(hand.cards, fn n -> get_rank(n) == h2 end)
			[h4 | _] = t3
			[h3] ++ [h4]
		else
			[h | _] = find_keys(hand.ranks, 2)
			Enum.filter(hand.cards, fn n -> get_rank(n) == h end)
		end
	end

	def flush(hand) do
		suits = Map.values(hand.suits)
		5 in suits or 6 in suits or 7 in suits
	end

	def get_flush(hand) do
		suit =
			Map.keys(hand.suits)
			|> Enum.max_by(fn n -> hand.suits[n] end)
		(Enum.filter(hand.cards, fn n -> (div(n-1, 13)+1) == suit end)
			|> Enum.sort_by(fn n -> get_rank(n) end)
			|> Enum.reverse()
			|> ace_sort()
			|> Enum.take(5))
	end

	# can find largest possible straight by sorting array in reverse
	def straight(hand) do
		cards = Enum.map(hand.cards, fn n -> get_rank(n) end)
				|> ace_sort
		ans = st(cards, 1)
		if elem(ans, 0) do
			{true, elem(ans, 1)}
		else
			{false, -1}
		end
	end

	def get_straight(straight, index, hand) do
		if (length straight) == 5 do
			Enum.sort(straight)
		else
			straight = straight ++
					(Enum.filter(hand.cards, fn n -> get_rank(n) == index end)
					|> Enum.take(1))
			if index == 13 do
				get_straight(straight, 1, hand)
			else
				get_straight(straight, index + 1, hand)
			end
		end
	end

	def is_three_of_a_kind(hand) do
		3 in Map.values(hand.ranks)
	end

	def get_three_of_a_kind(hand) do
		[h | _] = find_keys(hand.ranks, 3)
		trips = Enum.filter(hand.cards, fn n -> get_rank(n) == h end)
		ones  = Enum.filter(hand.cards, fn n -> get_rank(n) == 1 and get_rank(n) != h end)
		remaining =
			Enum.filter(hand.cards, fn n-> get_rank(n) != 1 and get_rank(n) != h end)
			|> Enum.sort_by(fn n-> get_rank(n) end)
			|> Enum.reverse()
		last_cards = Enum.take(ones ++ remaining, 2)
		trips ++ last_cards
	end

	# could potentially have 3 pairs hence >= 2 on line 216
	def is_two_pairs(hand) do
		counter = Enum.frequencies(Map.values(hand.ranks))
		counter[2] >= 2 and counter[2] != nil
	end

	def get_two_pairs(hand) do
		[h | t] = find_keys(hand.ranks, 2)
		[h2 | _] = t
		pairs =
			Enum.filter(hand.cards, fn n-> get_rank(n) == h end) ++
			Enum.filter(hand.cards, fn n-> get_rank(n) == h2 end)
		ones = Enum.filter(hand.cards,
				fn n -> get_rank(n) == 1 and get_rank(n) != h and get_rank(n) != h2 end)
		remaining =
			Enum.filter(hand.cards, fn n-> get_rank(n) != 1 and get_rank(n) != h and get_rank(n) != h2 end)
			|> Enum.sort_by(fn n-> get_rank(n) end)
			|> Enum.reverse()
		last_card = Enum.take(ones ++ remaining, 1)
		pairs ++ last_card
	end

	def is_pair(hand) do
		Enum.frequencies(Map.values(hand.ranks))[2] == 1
	end

	def get_pair(hand) do
		[h | _] = find_keys(hand.ranks, 2)
		pair = Enum.filter(hand.cards, fn n -> get_rank(n) == h end)
		ones = Enum.filter(hand.cards, fn n -> get_rank(n) == 1 and get_rank(n) != h end)
		remaining =
			Enum.filter(hand.cards, fn n -> get_rank(n) != h and get_rank(n) != 1 end)
			|> Enum.sort_by(fn n -> get_rank(n) end)
			|> Enum.reverse()
		last_cards = Enum.take(ones ++ remaining, 3)
		pair ++ last_cards
	end

	def high_card(hand) do
		ones = Enum.filter(hand.cards, fn n -> get_rank(n) == 1 end)
		remaining =
			Enum.filter(hand.cards, fn n -> get_rank(n) != 1 end)
			|> Enum.sort_by(fn n -> get_rank(n)  end)
			|> Enum.reverse()
		Enum.take(ones ++ remaining, 5)
	end

	def count_ranks(hand) do
		hand = Enum.map(hand, &get_rank/1)
		map = Enum.reduce(hand, %{}, fn char, acc ->
			Map.put(acc, char, (acc[char] || 0) + 1)
		end)
		map
	end

	def count_suits(hand) do
		hand = Enum.map(hand, fn(n) -> div(n-1, 13) + 1 end)
		map = Enum.reduce(hand, %{}, fn char, acc ->
			Map.put(acc, char, (acc[char] || 0) + 1)
		end)
		map
	end

	# turns number into string representation of card; ex:
	# 38 to '12H' (queen of hearts), 1 to '1C'
	def num_to_string(card) do
		rank = Integer.to_string(get_rank(card))
		suit = @suits[div(card-1, 13) + 1]
		rank <> suit
	end

	# if there is an ace, make sure it is used for high-card purposes
	def ace_sort(hand) do
		# [9 8 1]
		[h|t] = Enum.sort(hand, :asc)
		if h == 1 do
			[1 | Enum.sort(t, :desc)]
		else
			Enum.sort(hand, :desc)
		end
	end

	# helper functions for straight
	def st([h | _], 5) do
		{true, h}
	end
	def st([], _) do
		{false, -1}
	end
	def st([h | t], counter) do
		if h == 1 and List.first(t) == 13 do
			st(t, counter + 1)
		else
			if h - 1 == List.first(t)  do
				st(t, counter + 1)
			else
				st(t, 1)
			end
		end
	end

	# in order to rank King appropriately
	def get_rank(card) do
		if rem(card, 13) == 0 do
			13
		else
			rem(card, 13)
		end
	end

	def find_keys(map, value) do
		find_keys_helper([], Map.keys(map), map, value)
	end
	defp find_keys_helper(found, [], _, _) do
		[h|t] = Enum.sort(found, :asc)
		if h == 1 do
			[1 | Enum.sort(t, :desc)]
		else
			Enum.sort(found, :desc)
		end
	end
	defp find_keys_helper(found, keys, map, value) do
		[h | t] = keys
		found =
			if map[h] == value do
				[h | found]
			else
				found
			end
		find_keys_helper(found, t, map, value)
	end
end
