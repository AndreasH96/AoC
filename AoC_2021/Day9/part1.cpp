#include <iostream>
#include <regex>
#include <fstream>
#include "../../HelperFunctions/helperfunctions.h"
using namespace std;

std::vector<std::string> SplitString(
    std::string str,
    std::string delimeter)
{
    std::vector<std::string> splittedStrings = {};
    size_t pos = 0;

    while ((pos = str.find(delimeter)) != std::string::npos)
    {
        std::string token = str.substr(0, pos);
        if (token.length() > 0)
            splittedStrings.push_back(token);
        str.erase(0, pos + delimeter.length());
    }

    if (str.length() > 0)
        splittedStrings.push_back(str);
    return splittedStrings;
}

std::vector<std::string> readFileRows(std::string path)
{
    // Initialzie vector
    std::vector<std::string> rows;
    // Read from the text file
    std::ifstream MyReadFile(path);

    std::string myText;
    // Use a while loop together with the getline() function to read the file line by line
    while (std::getline(MyReadFile, myText))
    {
        rows.push_back(myText);
    }
    // Close the file
    MyReadFile.close();
    return rows;
}

vector<vector<int>> createHeightMap(string path)
{
    vector<vector<int>> heightMap;
    vector<string> fileRows = readFileRows(path);
    for (int row = 0; row < fileRows.size(); row++)
    {
        string currentRow = fileRows[row];
        vector<int> currentIntRow;
        cout << currentRow.size() << endl;
        for (int col = 0; col < currentRow.size(); col++)
        {
            currentIntRow.push_back(currentIntRow[col]);
            cout << "TEST";
        }
        heightMap.push_back(currentIntRow);
    }
    return heightMap;
}

int solve(string path)
{
    vector<vector<int>> heightMap = createHeightMap(path);

    for (int y = 0; y < heightMap.size(); y++)
    {
        for (int x = 0; x < heightMap[0].size(); x++)
        {
            cout << heightMap[y][x];
        }
    }
}

int main()
{
    cout << "SCORE: " << solve("C:/Users/ahaggstr/Documents/Programming/Personal/AoC/AoC_2021/Day9/testinput.txt");
}