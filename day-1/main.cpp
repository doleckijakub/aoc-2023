// https://adventofcode.com/2023/day/1

#include <iostream>
#include <fstream>
#include <cctype>

int main(int argc, const char **argv) {
	if(argc != 2) return 1;

	const char *program = argv[0];
	const char *filename = argv[1];

	std::ifstream input(filename);

	if(!input.is_open()) return 2;

	std::string line;
	int sum = 0;

	while(std::getline(input, line)) {
		for(int i = 0; i < line.length(); i++) if(isdigit(line[i])) {
			sum += 10 * (line[i] - '0'); break;
		}
		for(int i = line.length() - 1; i >= 0; i--) if(isdigit(line[i])) {
			sum += (line[i] - '0');
			break;
		}
	}

	std::cout << sum << std::endl;
}
