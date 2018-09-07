import { standard, siliconDawn, MajorArcana, Rank, Suit } from '../src/cards';

describe("decks", () => {
  it("should have the right number of cards in a standard deck", () => {
    expect(standard().length).toBe(22 + 4 * 14);
  });

  it("should start face-up", () => {
    expect(standard().filter((card) => card.reversed === false).length).toBe(78);
    expect(siliconDawn().filter((card) => card.reversed === false).length).toBe(94);
  });

  // 22 major arcana, 14 ranks in 4 suits each.
  const standardCount = 22 + 14 * 4;
  const ninetyNineCount = 4;
  const voidCount = 5;
  const extraCount = 7;
  describe("silicon dawn", () => {

    it("should default all options to true", () => {
      const noOptions = siliconDawn();
      const emptyOptions = siliconDawn({ voidSuit: true, ninetyNines: true, extraArcana: true });
      expect(noOptions).toEqual(emptyOptions);
    });

    it("should have the right number of cards with all default options", () => {
      expect(siliconDawn().length).toBe(standardCount + ninetyNineCount + voidCount + extraCount);
    });

    it("should have the right number of cards with no extras", () => {
      const options = { voidSuit: false, ninetyNines: false, extraArcana: false };
      expect(siliconDawn(options).length).toBe(standardCount);
    });

    it("should have the right number of cards with the void suit", () => {
      const options = { voidSuit: true, ninetyNines: false, extraArcana: false };
      expect(siliconDawn(options).length).toBe(standardCount + voidCount);
    });

    it("should have the right number of cards with the 99s", () => {
      const options = { voidSuit: false, ninetyNines: true, extraArcana: false };
      expect(siliconDawn(options).length).toBe(standardCount + ninetyNineCount);
    });

    it("should have the right number of cards with the extra arcana", () => {
      const options = { voidSuit: false, ninetyNines: false, extraArcana: true };
      expect(siliconDawn(options).length).toBe(standardCount + extraCount);
    });

  });
});

describe("the major arcana", () => {
  it("should have the right count of standard arcana", () => {
    expect(MajorArcana.standard().length).toBe(22);
  });

  it("should have the right count of Silicon Dawn arcana", () => {
    expect(MajorArcana.siliconDawn().length).toBe(5);
  })
});

describe("the ranks", () => {
  it("should have the right count of standard ranks", () => {
    expect(Rank.standard().length).toBe(14);
  });
});

describe("the suits", () => {
  it("should have the right count of standard suits", () => {
    expect(Suit.standard().length).toBe(4);
  });
});
