#ifndef HELPERFUNCTIONS_H    // To make sure you don't declare the function more than once by including the header multiple times.
#define HELPERFUNCTIONS_H

#include <vector>
#include <string>


std::vector<std::string> SplitString(
    std::string str,
    std::string delimeter);

std::vector<std::string> readFileRows(std::string path);

std::string readWholeFile(std::string path);
#endif