#include <fstream>
#include <iostream>

int main()
{
  std::ifstream in("day1/input.txt");

  if (!in)
  {
    std::cerr << "Can not open file!";
    return EXIT_FAILURE;
  }

  int frequency = 0;
  //std::string line;
  //while (std::getline(in, line)) {
  for( std::string line; std::getline( in, line ); ) {
    if (line.find("+") == 0) {
      frequency += stoi(line.substr(1, line.length()));
    } else {
      frequency -= stoi(line.substr(1, line.length()));
    }
  }
  std::cout << "Frequency: " << frequency;

  return EXIT_SUCCESS;
}
