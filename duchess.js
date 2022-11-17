import { best_move } from './pkg/duchesslib.js';

var board = null

function duchessMove(fromFEN) {
  var toFEN = best_move(fromFEN)
  board.position(toFEN)
}

function onDrop (source, target, piece, newPos, oldPos, orientation) {
  // console.log('Source: ' + source)
  // console.log('Target: ' + target)
  // console.log('Piece: ' + piece)
  // console.log('New position: ' + Chessboard.objToFen(newPos))
  // console.log('Old position: ' + Chessboard.objToFen(oldPos))
  // console.log('Orientation: ' + orientation)
  // console.log('~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~')

  var player = piece[0] == 'w' ? 'b' : 'w'
  var fen = Chessboard.objToFen(newPos) + ' ' + player + ' KQkq'
  window.setTimeout(duchessMove, 100, fen)
}

// function onMoveEnd (oldPos, newPos) {
//   console.log('Move animation complete:')
//   console.log('Old position: ' + Chessboard.objToFen(oldPos))
//   console.log('New position: ' + Chessboard.objToFen(newPos))
//   console.log('~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~')
// }

export function duchess () {
  var config = {
    draggable: true,
    position: 'start',
    orientation: 'white',
    // onDragStart: onDragStart,
    onDrop: onDrop,
    // onSnapEnd: onSnapEnd
    // onMoveEnd: onMoveEnd
  }
  board = Chessboard('board1', config)
}
