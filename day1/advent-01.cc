#include <fstream>
#include <iostream>

int main()
{
  std::ifstream in("01/input.txt");

  if (!in)
  {
    std::cerr << "Can not open file!";
    return EXIT_FAILURE;
  }

  std::string message;

  if (!(in >> message))
  {
    std::cerr << "Can not read file content!";
    return EXIT_FAILURE;
  }

  // Begin computing frequency.
  

  return EXIT_SUCCESS;
}
