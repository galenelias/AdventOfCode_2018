#include <iostream>
#include <algorithm>
#include <list>
#include <vector>
#include <fstream>
#include <string>
#include <set>
#include <cassert>

using namespace std;

enum class Direction {
	Left, Up, Down, Right
};

struct Cart {
	int r, c;
	Direction direction;
	int turn_preference; // left, straight, right	
};

Direction turn_left(Direction dir) {
	switch (dir) {
	case Direction::Up: return Direction::Left;
	case Direction::Right: return Direction::Up;
	case Direction::Down: return Direction::Right;
	case Direction::Left: return Direction::Down;
	}
}

Direction turn_right(Direction dir) {
	switch (dir) {
	case Direction::Up: return Direction::Right;
	case Direction::Right: return Direction::Down;
	case Direction::Down: return Direction::Left;
	case Direction::Left: return Direction::Up;
	}
}



int main()
{
	vector<string> grid;
	string temp;

	while (cin)
	{
		std::getline(cin, temp);
		if (cin)
		{
			cout << temp << "\n";
			grid.push_back(std::move(temp));

		}
	}

	vector<Cart> carts;

	for (int r = 0; r < grid.size(); ++r) {
		for (int c = 0; c < grid[r].size(); ++c) {
			auto ch = grid[r][c];
			if (ch == '<') {
				carts.push_back(Cart{ r, c, Direction::Left, 0 });
				grid[r][c] = '-';
			}
			else if (ch == '>') {
				carts.push_back(Cart{ r,  c, Direction::Right, 0 });
				grid[r][c] = '-';
			}
			else if (ch == '^') {
				carts.push_back(Cart{ r, c, Direction::Up, 0 });
				grid[r][c] = '|';
			}
			else if (ch == 'v') {
				carts.push_back(Cart{ r,  c, Direction::Down, 0 });
				grid[r][c] = '|';
			}
		}
	}

	for ( int iteration = 0; ; ++iteration) {
		//cout << "Iteration " << iteration << ": " << carts.size() << "\n";

		set<pair<int, int>> positions;

		sort(carts.begin(), carts.end(), [](const Cart& a, const Cart& b) {
			if (a.r < b.r)
				return true;
			else if (a.r > b.r)
				return false;
			else
				return a.c < b.c;
		});


		// for cart in &mut carts {
		for (int i = 0; i < carts.size(); ++i) {
			Cart& cart = carts[i];

			if (cart.direction == Direction::Up) {
				--cart.r;
			} else if (cart.direction == Direction::Down) {
				++cart.r;
			} else if (cart.direction == Direction::Left) {
				--cart.c;
			} else if (cart.direction == Direction::Right) {
				++cart.c;
			}

			bool crashed = false;
			for (int j = 0; j < carts.size(); ++j) {
				if (i != j && carts[i].r == carts[j].r && carts[i].c == carts[j].c) {
					//cout << "Crash!  " << i << ", " << j << " - " << carts[i].c << ", " << carts[i].r << "\n";
					int a = min(i, j);
					int b = max(i, j);

					crashed = true;
					carts.erase(carts.begin() + a);
					carts.erase(carts.begin() + b - 1);

					if (i == a) {
						--i;
					}
					else {
						i -= 2;
					}
				}
			}
			if (crashed)
				continue;

			auto ch = grid[cart.r][cart.c];
			if (ch == '+') {
				if (cart.turn_preference == 0) {
					cart.direction = turn_left(cart.direction);
				}
				else if (cart.turn_preference == 1) {
					//cart.direction = go_straight(&cart.direction);
				}
				else if (cart.turn_preference == 2) {
					cart.direction = turn_right(cart.direction);
				}
				cart.turn_preference = (cart.turn_preference + 1) % 3;
			}
			else if (ch == '/') {
				if (cart.direction == Direction::Up) {
					cart.direction = Direction::Right;
				} else if (cart.direction == Direction::Right) {
					cart.direction = Direction::Up;
				} else if (cart.direction == Direction::Down) {
					cart.direction = Direction::Left;
				} else if (cart.direction == Direction::Left) {
					cart.direction = Direction::Down;
				}
			}
			else if (ch == '\\') {
				if (cart.direction == Direction::Up) {
					cart.direction = Direction::Left;
				} else if (cart.direction == Direction::Left) {
					cart.direction = Direction::Up;
				} else if (cart.direction == Direction::Down) {
					cart.direction = Direction::Right;
				} else if (cart.direction == Direction::Right) {
					cart.direction = Direction::Down;
				}
			}
		}

		if (carts.size() == 1) {
			cout << "Part 2: " << carts[0].c << ", " << carts[0].r << "\n";
			break;
		}
	}
}