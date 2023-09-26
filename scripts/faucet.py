from requests import get, post
from hashlib import sha256

URL = "http://0.0.0.0:5001/api/v1/faucet"

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
        'target': 'atest1d9khqw36x4pr2sfkgv6njv2rxuunjvpsxfzr2w29x5crxvfhxsurwdpn8qm5yd2xgverj3336str0x',
        'token': 'atest1v4ehgw36x3prswzxggunzv6pxqmnvdj9xvcyzvpsggeyvs3cg9qnywf589qnwvfsg5erg3fkl09rg5',
        'amount': 100
    }
})

print(response.json())