// Weight (exercise load, bodyweight) is always stored as kilograms. These
// helpers convert to/from the person's display unit at the UI boundary only.

const KG_PER_LB = 0.45359237;

export const KILOGRAMS = 'Kilograms';
export const POUNDS = 'Pounds';

// The unit conventionally associated with a language, used only until the
// person makes an explicit choice in Settings.
export const getDefaultWeightUnit = (language) => {
    return language?.toLowerCase().startsWith('en') ? POUNDS : KILOGRAMS;
};

// Resolves the unit weight should be displayed/entered in: the explicit
// `weightUnit` setting if the person picked one, else the language default.
export const resolveWeightUnit = (weightUnit, language) => {
    return weightUnit || getDefaultWeightUnit(language);
};

export const weightUnitLabel = (unit) => (unit === POUNDS ? 'lbs' : 'kg');

export const kgToDisplay = (kg, unit) => (unit === POUNDS ? kg / KG_PER_LB : kg);

export const displayToKg = (value, unit) => (unit === POUNDS ? value * KG_PER_LB : value);
