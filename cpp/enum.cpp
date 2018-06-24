#include <iostream> //For standard I/O
using namespace std; //Reserve library objects for standard uses
enum WeekDay {Mon, Tue, Wed, Thu, Fri}; //An enumeration type
int main()
{
 // WeekDay d; //An enumeration variable
 // for (d = Mon; d <= Fri; d = WeekDay(d+1)) //Casting the updated index
 for (auto d = Mon; d <= Fri; d = (d+1)) //Casting the updated index
cout << d << " "; //Output 0 to 4 instead of Mon to Fri
 return 0; //Successfully done
}