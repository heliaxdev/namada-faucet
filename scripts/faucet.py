from requests import get, post
from hashlib import sha256

URL = "http://0.0.0.0:5000/api/v1/faucet"

def request_challenge():
    return get(URL).json()

def request_transfer(data):
    return post(URL, json=data)

def is_valid_pow(solution, difficulty):
    for i in range(0, difficulty):
        if solution[i] != 0:
            return False
    return True

def compute_pow_solution(challenge, difficulty):
    challenge_bytes = bytes.fromhex(challenge)
    i = 0
    while i >= 0:
        hasher = sha256()
        hasher.update(challenge_bytes)
        hasher.update(i.to_bytes(64, byteorder='big'))
        hash = hasher.digest()

        valid = is_valid_pow(hash, difficulty)
        if valid:
            return i.to_bytes(64, byteorder='big').hex()
        i += 1

response = request_challenge()

solution = compute_pow_solution(response['challenge'], 2)

response = request_transfer({
    'solution': solution,
    'tag': response['tag'],
    'challenge': response['challenge'],
    'transfer': {
        'target': 'tnam1qyf2tv9l8w7hfu5glr72chzz5zjknywjugnvh34x',
        'token': 'tnam1qyxg9d8zr4su9ks3a368485kjjr3e880kysz076g',
        'amount': 100 * 10**6
    }
})

print(response.json())