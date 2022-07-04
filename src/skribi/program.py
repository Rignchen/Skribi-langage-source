#!/usr/bin/env python3
# *-* coding:utf-8 *-*

# ================== #
# class of a program #
# ================== #
from src.skribi.skribi_file import SkribiFile
from src.skribi.tokens import Lexer
from src.skribi.custom_exception import SkribiException
from src.skribi.parser import Parser


class Program:

    def __init__(self):
        self.files = []

    def analyse(self, file: SkribiFile) -> None:
        """
        Execute the file
        :param file:
        :return:
        """

        self.files.append(file)

        # first step: transform the file into a list of tokens
        file.lexer = Lexer(file.get_content(), file)
        tokens = []
        for token in file.lexer:
            if isinstance(token, SkribiException):
                return
            tokens.append(token)

        # second step: analyse the list of tokens
        file.parser = Parser()
        file.result = file.parser.parse(tokens)
        if isinstance(file.result, SkribiException):
            file.result.print_complete_error()
            return

        # third step: execute the program
        print(file.result.evaluate())
