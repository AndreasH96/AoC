#include<iostream>
#include<regex>
#include"armadillo"
#include "../../HelperFunctions/helperfunctions.h"

using namespace std;
using namespace arma;
arma::mat createBingoBoard(string bingoString)
{   
    arma::mat board(5,5);
    string bingoStringFiltered = regex_replace(bingoString, regex("\n"), " ");
    vector<string> bingoSplit = SplitString(bingoStringFiltered, " ");
    for (int i = 0; i < 5; i++)
    {
        for (int j = 0; j < 5; j++)
        {
            board(i,j) = stoi(bingoSplit[(i*5) + j]); 
        }
    }
    return board;
}
vector<int> formatDraws(string drawsString){
    vector<int> draws;
    vector<string> drawsSplit = SplitString(drawsString,",");

    for (int i = 0; i < drawsSplit.size(); i++)
    {
       draws.push_back(stoi(drawsSplit[i]));
    }
    
    return draws;
    
}
vector<arma::mat> createBoards(vector<string> contentData){
    vector<arma::mat> boards;
    for (int i = 1; i < contentData.size(); i++)
    {
       boards.push_back(createBingoBoard(contentData[i]));
    }
    return boards;
}

void performDraw(int draw,vector<arma::mat> &boards){
    for (auto &&board : boards)
    {
        for (int row = 0; row < board.n_rows; row++)
        {
            for (int column = 0; column < board.n_cols; column++)
            {
                board(row,column)  = (board(row,column) == draw) ? -1  : board(row,column);
            }
        }
    }
}

int calcSum(vector<int> vec){
    int sum = 0;
    for (auto &&i : vec)
    {
        sum+=i;    
    }
    return sum;
}
int bingoSum(mat matrix){
    uvec rowSum = arma::find(arma::sum(matrix,0) == -5);
    uvec colSum = arma::find(arma::sum(matrix,1) == -5);
    if((!rowSum.empty()) || (!colSum.empty())) {
        matrix.transform( [](double val) { return (val < 0 ? 0 : val); } );
        int sum =(int) arma::sum(arma::sum(matrix));
        return sum;
    }
    return 0;

}


int solve(string path){
    string fileContent = readWholeFile(path);
    vector<string> content =SplitString(fileContent,"\n\n");
    vector<int> draws = formatDraws(content[0]);
    vector<arma::mat> boards = createBoards(content);
    int winners;


    int bingo =0;
    
    for (int drawNumber = 0; drawNumber < draws.size(); drawNumber++)
    {
        /* code */
        performDraw(draws[drawNumber],boards);
        for (int boardNumber = 0; boardNumber < boards.size(); boardNumber++)
        {
            bingo = bingoSum(boards[boardNumber]);
            if(bingo > 0){
                if(boards.size() == 1){
                    return draws[drawNumber] * bingo;
                }
                boards.erase(boards.begin()+boardNumber);
            }

        }      
    } 
    return 0; 

}

int main() {
    cout << "SCORE: " << solve("input.txt");      
}