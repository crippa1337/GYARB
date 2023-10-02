# Universal Ataxx Interface (Inspired by Universal Chess Interface)
from subprocess import Popen, PIPE
from time import sleep

class Engine:
    name: str
    author: str
    options: dict[str, str]
    def __init__(self, name, author, options={}):
        self.name = name
        self.author = author
        self.options = options


class UAIUnexpectedCommand(Exception):
    """Raised when the engine sends an invalid or unexpected command"""
    pass

class Handler:
    engine: Engine | None
    process: Popen
    def __init__(self, path, **kwargs):
        self.process = Popen([path, kwargs], stdin=PIPE, stdout=PIPE, universal_newlines=True)
        self.engine = None
    

    def read(self) -> str:
        return self.process.stdout.readline().decode("utf-8")
    
    def write(self, input):
        data = bytes(input + '\n', "utf-8")
        self.process.stdin.write(data)
    
    def identify(self):
        self.write("uci")
        name, author = None
        options = dict()
        while 1:
            # Recieve line and split whitespace
            recv = self.read().split()
            command = recv[0]
            if command == "id":
                value = ' '.join(recv[2::])
                if recv[1] == "name":
                    name = value
                elif recv[1] == "author":
                    author = value
                else:
                    raise UAIUnexpectedCommand(f"Did not expect '{recv[1]}' after 'id'")
            elif command == "option":
                value = ' '.join(recv[2::])
                options[recv[1]] = value
            elif command == "uciok":
                break
            else:
                raise UAIUnexpectedCommand(f"Did not expect '{command}' in the identification phase")
        self.engine = Engine(name, author, options=options)
    
    def run():
        pass