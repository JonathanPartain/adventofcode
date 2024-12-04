#!/usr/bin/env escript

main(_) ->
    Input = get_input(),
    io:format("~p\n", [solution_1(Input)]),
    io:format("~p\n", [solution_2(Input)]).

get_input() ->
    {ok, RawData} = file:read_file("input.txt"),
    Raw = binary:split(RawData, <<"\n">>, [global, trim]),
    [ begin
          [L,R] = binary:split(Line, <<" ">>, [global,trim_all]),
          {binary_to_integer(L),
           binary_to_integer(R)}
    end || Line <- Raw].
    

% We stealing today
% https://erlangforums.com/t/advent-of-code-2024-day-1/4231/4

solution_1(Input) ->
    {LL, RL} = lists:unzip(Input),
    lists:sum([ abs( L - R) || {L, R} <- lists:zip(lists:sort(LL), lists:sort(RL))]).

count(_, []) -> 0;
count(X, [X|XS]) -> 1 + count(X, XS);
count(X, [_|XS]) -> count(X, XS).

solution_2(Input) ->
    {LL, RL} = lists:unzip(Input),
    V = maps:groups_from_list(fun(I) -> I end, RL),
    lists:sum([A * length(maps:get(A, V, [])) || A<- LL]).
