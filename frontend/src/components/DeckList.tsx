import * as React from "react";
import * as Cards from "../cards/cards";
import ReactModal = require('react-modal');
import { NewDeckDialog } from './NewDeckDialog';

interface Props {
  decks: ReadonlyArray<Cards.Deck>,
  // Called whenever the user created a new deck.
  onNewDeck: (deck: Cards.Deck) => void,
  // Called whenever the user wants to delete the deck with the given index.
  onDeleteDeck: (index: number) => void,
}

interface State {
  showNewDeckDialog: boolean
}

export default class DeckList extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { showNewDeckDialog: false };

    this.handleNewDeckButton = this.handleNewDeckButton.bind(this);
    this.handleNewDeck = this.handleNewDeck.bind(this);
  }

  handleNewDeckButton(e: React.FormEvent) {
    this.setState({ showNewDeckDialog: true });
  }

  handleNewDeck(deck: Cards.Deck) {
    this.setState({ showNewDeckDialog: false });
    this.props.onNewDeck(deck);
  }

  render() {
    const deckItems = this.props.decks.map((deck, index) =>
      <li key={deck.id}>
        {deck.name}
        <button onClick={() => this.props.onDeleteDeck(index)}>Delete</button>
      </li>);
    return (
      <div>
        <ReactModal
          isOpen={this.state.showNewDeckDialog}
          className="modal"
          overlayClassName="overlay"
          closeTimeoutMS={200}>
          <NewDeckDialog onNewDeck={this.handleNewDeck} />
        </ReactModal>
        <ul>{deckItems}</ul>
        <button onClick={this.handleNewDeckButton}>Add new deck</button>
      </div>
    );
  }
}
