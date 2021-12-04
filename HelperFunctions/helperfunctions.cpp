#include <vector>
#include <string>
#include <iostream>
#include <fstream>
#include "helperfunctions.h"

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

std::vector<std::string> readFileRows(std::string path){
    // Initialzie vector
    std::vector<std::string> rows;
    // Read from the text file
    std::ifstream MyReadFile(path);

    std::string myText;
    // Use a while loop together with the getline() function to read the file line by line
    while (std::getline (MyReadFile, myText)) {
        rows.push_back(myText);
    }
    // Close the file
    MyReadFile.close();  
    return rows;
}

std::string readWholeFile(std::string path){
    // Initialzie vector
    std::string file_content;
    // Read from the text file
    std::ifstream MyReadFile(path);
    // Use a while loop together with the getline() function to read the file line by line
    std::getline(MyReadFile, file_content, '\0');
    // Close the file
    MyReadFile.close();  
    return file_content;
}
