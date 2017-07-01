let numberOfPoints = process.env.NUMBER_OF_POINTS || 100
for(var i=0; i < numberOfPoints; i++) {
  console.log(Math.min(36000, generateRandomPoint()) + ' ' + Math.min(18000, generateRandomPoint()))
}

function generateRandomPoint() {
	return Math.ceil(Math.random() * Math.random() * 50000)
}
