#include <map>
#include <vector>
#include <iostream>
#include <armadillo>
#include "../../HelperFunctions/helperfunctions.h"
using namespace std;
using namespace arma;

int main() {
    using direction = std::tuple<int,int>;
    direction forward = {1,0};
    direction down = {0,1};
    direction up = {0,-1};
    map<string, direction> directions = {
        { "forward", forward },
        { "down", down },
        { "up", up }
    };

    std::vector<string> rows = readFileRows("testinput.txt");
    for (int i = 0; i < rows.size(); i++)
    {
        string currentD = SplitString(rows[i]," ")[0];
        
        cout << currentD;
        cout << std::get<0>(directions[currentD]); 
        mat A(4, 5, fill::randu);
        mat B(4, 5, fill::randu);
  
        cout << A*B.t() << endl;
  
        /* code */
    }
  
}