#include "iostream"
using namespace std;

int main (int argc, char *argv[]) {
  
  string food = "Pizza";
  string &meal = food;

  cout << food << "\n";  // Outputs Pizza
  cout << meal << "\n";  // Outputs Pizza

  string* ptr = &food;
  cout << ptr << "\n"; // Output memory
  cout << *ptr << "\n";

  return 0;
}
