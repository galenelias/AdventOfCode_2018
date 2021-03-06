#include <iostream>
#include <algorithm>
#include <list>
#include <vector>

void advance_iterator(std::list<uint64_t>& container, std::list<uint64_t>::iterator& iter, size_t amount)
{
	for (size_t i = 0; i < amount; ++i)
	{
		if (iter == container.end())
			iter = container.begin();
		++iter;
	}
}

void rewind_iterator(std::list<uint64_t>& container, std::list<uint64_t>::iterator& iter, size_t amount)
{
	for (size_t i = 0; i < amount; ++i)
	{
		if (iter == container.begin())
			iter = container.end();
		--iter;
	}
}

void run_game(size_t players, uint64_t last_marble_points)
{
	std::list<uint64_t> marbles = { 0 };
	std::vector<uint64_t> scores(players, 0);

	auto iter = marbles.begin();
	for (uint64_t marble_number = 1; marble_number <= last_marble_points; ++marble_number)
	{
		if (marble_number % 23 == 0)
		{
			rewind_iterator(marbles, iter, 7);
			scores[(marble_number - 1) % players] += marble_number + *iter;
			iter = marbles.erase(iter);
		}
		else
		{
			advance_iterator(marbles, iter, 2);
			iter = marbles.insert(iter, marble_number);
		}
	}

	auto maxElement = max_element(scores.begin(), scores.end());
	std::cout << "Players: " << players << "\t\tLast marble: " <<  last_marble_points << "\t\tHigh Score: " << *maxElement << "\n";
}

int main()
{
	run_game(455, 71223);
	run_game(455, 7122300);
}

