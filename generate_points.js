let numberOfPoints = process.env.NUMBER_OF_POINTS || 100
for(var i=0; i < numberOfPoints; i++) {
  console.log(Math.min(3600, generateRandomPoint()) + ' ' + Math.min(1800, generateRandomPoint()))
}

function generateRandomPoint() {
	return Math.ceil(Math.random() * 4000)
}
