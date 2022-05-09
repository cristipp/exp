function main() {
  const jwt = require('jsonwebtoken');
  const CUBE_API_SECRET = 'SECRET';

  const cubejsToken = jwt.sign({}, CUBE_API_SECRET, { expiresIn: '30y' });
  console.log('token:', cubejsToken);
}

main()
