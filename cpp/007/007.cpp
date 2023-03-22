// nullptr
#include <iostream>
#include <new>
using namespace std;
int main ()
{
  int i = 0;
  int n = 0;
  int * p;
  cout << "How many numbers would you like to type? ";
  cin >> i;

  p = new (nothrow) int[i];
  if (p == nullptr)
    cout << "Error: memory could not be allocated";
  else
  {
    for (n=0; n<i; n++)
    {
      cout << "Enter number: ";
      cin >> p[n];
    }
    cout << "You have entered: ";
    for (n=0; n<i; n++)
      cout << p[n] << ", ";
    delete[] p;
  }
  return 0;
}
