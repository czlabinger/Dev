// function template
#include <iostream>
using namespace std;

template <class T>
T sum (T a, T b)
{
  return a + b;
}

int main () {
  int k = sum<int>(1, 2);
  double h = sum<double>(2.5, 2.5);
  cout << k << '\n';
  cout << h << '\n';
  return 0;
}
