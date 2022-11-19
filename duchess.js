import { make_move, best_move } from './pkg/duchesslib.js';

var board = null
var $last_human = $('#last_human')
var $last_duchess = $('#last_duchess')

var last_human_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
var last_duchess_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"

function duchessMove(fromFEN) {
  last_duchess_fen = best_move(fromFEN)
  board.position(last_duchess_fen)
  $last_duchess.html(last_duchess_fen)
}

function onDrop (source, target, piece, newPos, oldPos, orientation) {
  // make move returns a new FEN, or 'illegal'
  var fen = make_move(last_duchess_fen, source, target)
  if(fen == 'illegal') {
    return 'snapback'
  }
  last_human_fen = fen
  $last_human.html(last_human_fen)
  window.setTimeout(duchessMove, 100, last_human_fen)
}


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
