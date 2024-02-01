#include "iostream"
#include <string>
using namespace std;


int main (int argc, char *argv[]) {
  
  string input;
  
  cout << "Type a string" << "\n";
  getline(cin, input);
  cout << input;

  return 0;
}
